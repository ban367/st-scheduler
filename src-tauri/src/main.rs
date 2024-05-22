// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;
use std::fs;
use tauri::api::path::download_dir;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            write_file,
            analyze_calendar_data,
            analyze_calendar_data_with_st
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserAggregate {
    attendance: u32,
    #[serde(rename = "stRewards", default)]
    st_rewards: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CalendarDay {
    #[serde(rename = "userIds")]
    user_ids: Vec<u32>,
    #[serde(rename = "stUserIds")]
    st_user_ids: Vec<u32>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CalendarData {
    year: u32,
    month: u32,
    days: HashMap<String, CalendarDay>,
    #[serde(rename = "userAggregate", default)]
    user_aggregate: HashMap<u32, UserAggregate>,
}

fn aggregate_user_data(
    attendance_count: &HashMap<u32, u32>,
    st_count: &HashMap<u32, u32>,
) -> HashMap<u32, UserAggregate> {
    let mut aggregate_data = HashMap::new();
    for (&user_id, &attendance) in attendance_count {
        let st_rewards = *st_count.get(&user_id).unwrap_or(&0);
        let valid_st_rewards = if st_rewards > attendance {
            attendance
        } else {
            st_rewards
        };
        aggregate_data.insert(
            user_id,
            UserAggregate {
                attendance,
                st_rewards: valid_st_rewards,
            },
        );
    }
    aggregate_data
}

#[tauri::command]
async fn write_file(filename: &str, content: &str) -> Result<String, String> {
    let mut path = download_dir().ok_or("Could not find download directory")?;
    path.push(filename);

    fs::write(&path, content).map_err(|err| err.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

fn calculate_attendance(data: &CalendarData) -> HashMap<u32, u32> {
    let mut attendance_count = HashMap::new();
    for day in data.days.values() {
        for &user_id in &day.user_ids {
            *attendance_count.entry(user_id).or_insert(0) += 1;
        }
    }
    attendance_count
}

fn initialize_user_assignments(
    attendance_count: &HashMap<u32, u32>,
    data: &CalendarData,
    enforce_st_user_ids: bool,
) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
    let mut user_assignments = HashMap::new();
    let mut st_count = HashMap::new();
    for (user_id, count) in attendance_count {
        let assignments = if *count >= 12 {
            6
        } else if *count >= 6 {
            4
        } else if *count >= 1 {
            2
        } else {
            0
        };
        user_assignments.insert(*user_id, assignments);
        st_count.insert(*user_id, 0);
    }

    if enforce_st_user_ids {
        for day in data.days.values() {
            for &user_id in &day.st_user_ids {
                if let Some(assignments) = user_assignments.get_mut(&user_id) {
                    let new_assignments = (*assignments as u32).saturating_sub(1);
                    *assignments = new_assignments;
                }
                if let Some(count) = st_count.get_mut(&user_id) {
                    *count += 1;
                }
            }
        }
    }

    (user_assignments, st_count)
}

fn assign_st_rewards(
    data: &CalendarData,
    exclude_user_ids: &[u32],
    exclude_dates: &[u32],
    available_st: u32,
    user_assignments: &mut HashMap<u32, u32>,
    st_count: &mut HashMap<u32, u32>,
    enforce_st_user_ids: bool,
) -> HashMap<String, CalendarDay> {
    let mut new_days = HashMap::new();
    for (date, day) in &data.days {
        let date_key = date.parse::<u32>().unwrap_or(0);
        if exclude_dates.contains(&date_key) {
            new_days.insert(
                date.clone(),
                CalendarDay {
                    user_ids: day.user_ids.clone(),
                    st_user_ids: if enforce_st_user_ids {
                        day.st_user_ids.clone()
                    } else {
                        Vec::new()
                    },
                },
            );
            continue;
        }

        let mut available_st = available_st;
        let mut day_rewards = if enforce_st_user_ids {
            day.st_user_ids.clone()
        } else {
            Vec::new()
        };

        if enforce_st_user_ids {
            for &user_id in &day.st_user_ids {
                if !exclude_user_ids.contains(&user_id)
                    && user_assignments.get(&user_id).unwrap_or(&0) > &0
                {
                    if let Some(assignments) = user_assignments.get_mut(&user_id) {
                        let new_assignments = (*assignments).saturating_sub(1);
                        *assignments = new_assignments;
                    }
                    if let Some(count) = st_count.get_mut(&user_id) {
                        *count += 1;
                    }
                    available_st = available_st.saturating_sub(1);
                }
            }
        }

        let mut sorted_user_ids = day.user_ids.clone();
        sorted_user_ids
            .sort_by_key(|&id| std::cmp::Reverse(*user_assignments.get(&id).unwrap_or(&0)));
        for &user_id in &sorted_user_ids {
            if !exclude_user_ids.contains(&user_id)
                && available_st > 0
                && user_assignments.get(&user_id).unwrap_or(&0) > &0
            {
                if !day_rewards.contains(&user_id) {
                    day_rewards.push(user_id);
                    if let Some(assignments) = user_assignments.get_mut(&user_id) {
                        let new_assignments = (*assignments).saturating_sub(1);
                        *assignments = new_assignments;
                    }
                    *st_count.get_mut(&user_id).unwrap() += 1;
                    available_st = available_st.saturating_sub(1);
                }
            }
        }
        new_days.insert(
            date.clone(),
            CalendarDay {
                user_ids: day.user_ids.clone(),
                st_user_ids: day_rewards,
            },
        );
    }
    new_days
}

#[tauri::command]
fn analyze_calendar_data_with_st(
    data: CalendarData,
    exclude_user_ids: Vec<u32>,
    exclude_dates: Vec<u32>,
    available_st: u32,
) -> CalendarData {
    let attendance_count = calculate_attendance(&data);
    let (mut user_assignments, mut st_count) =
        initialize_user_assignments(&attendance_count, &data, true);
    let new_days = assign_st_rewards(
        &data,
        &exclude_user_ids,
        &exclude_dates,
        available_st,
        &mut user_assignments,
        &mut st_count,
        true,
    );
    let user_aggregate = aggregate_user_data(&attendance_count, &st_count);

    CalendarData {
        year: data.year,
        month: data.month,
        days: new_days,
        user_aggregate,
    }
}

#[tauri::command]
fn analyze_calendar_data(
    data: CalendarData,
    exclude_user_ids: Vec<u32>,
    exclude_dates: Vec<u32>,
    available_st: u32,
) -> CalendarData {
    let attendance_count = calculate_attendance(&data);
    let (mut user_assignments, mut st_count) =
        initialize_user_assignments(&attendance_count, &data, false);
    let new_days = assign_st_rewards(
        &data,
        &exclude_user_ids,
        &exclude_dates,
        available_st,
        &mut user_assignments,
        &mut st_count,
        false,
    );
    let user_aggregate = aggregate_user_data(&attendance_count, &st_count);

    CalendarData {
        year: data.year,
        month: data.month,
        days: new_days,
        user_aggregate,
    }
}
