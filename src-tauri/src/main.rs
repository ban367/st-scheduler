// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_calendar_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
}

#[tauri::command]
fn process_calendar_data(data: CalendarData) -> CalendarData {
    let mut attendance_count = HashMap::new();

    // 出席回数をカウント
    for day in data.days.values() {
        for &user_id in &day.user_ids {
            *attendance_count.entry(user_id).or_insert(0) += 1;
        }
    }

    let mut user_assignments = HashMap::new();
    for (user_id, count) in &attendance_count {
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
    }

    let mut new_days = HashMap::new();
    for (date, day) in &data.days {
        let mut available_st = 7; // 1日に割り当て可能なSTの最大数
        let mut day_rewards = Vec::new();
        let mut sorted_user_ids = day.user_ids.clone();
        // 割り当てる報酬の多い順にユーザーをソート
        sorted_user_ids.sort_by_key(|&id| user_assignments.get(&id).unwrap_or(&0));
        for &user_id in &sorted_user_ids {
            if available_st > 0 && user_assignments.get(&user_id).unwrap_or(&0) > &0 {
                day_rewards.push(user_id);
                *user_assignments.get_mut(&user_id).unwrap() -= 1;
                available_st -= 1;
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

    CalendarData {
        year: data.year,
        month: data.month,
        days: new_days,
    }
}
