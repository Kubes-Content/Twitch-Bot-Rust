use crate::save_data::default::user_rpg_stats::UserRpgStats;
use crate::user::user_properties::UserId;
use std::collections::HashMap;

const TICKS_PER_EXP_POINT: u32 = 40;

#[derive(Default)]
pub struct TickData {
    active_viewers_data: HashMap<UserId, u32>, // consecutive ticks
}

impl TickData {
    pub fn tick_on_users(&mut self, channel: UserId, current_active_viewers_ids: Vec<UserId>) {
        self.remove_missing_viewers(current_active_viewers_ids.clone());

        self.update_specific_viewers_data(channel, current_active_viewers_ids);
    }

    fn remove_missing_viewers(&mut self, current_active_viewers_ids: Vec<UserId>) {
        let mut keys_to_remove = Vec::new();

        // gather missing users
        for key in self.active_viewers_data.keys() {
            if !current_active_viewers_ids.contains(key) {
                keys_to_remove.push(key.clone());
            }
        }

        // remove missing users
        for key in keys_to_remove {
            self.active_viewers_data.remove(&key);
        }
    }

    fn update_specific_viewers_data(
        &mut self,
        channel: UserId,
        current_active_viewers_ids: Vec<UserId>,
    ) {
        let previous_active_viewer_ids: Vec<UserId> =
            self.active_viewers_data.keys().map(|u| u.clone()).collect();

        // update present users
        for user_id in current_active_viewers_ids {
            //
            // insert new viewer
            if !previous_active_viewer_ids.contains(&user_id) {
                green_ln!("Adding new viewer exp tick entry: {}", user_id.get_value());
                self.active_viewers_data.insert(user_id.clone(), 1);

                return;
            }

            //
            // update existing viewer

            green_ln!(
                "Updating existing viewer's exp tick data: id:{}",
                user_id.get_value()
            );

            let existing_viewer_experience = match self.active_viewers_data.get(&user_id) {
                Some(x) => *x,
                None => {
                    println!(
                        "ERROR TRYING TO UPDATE CHARACTER'S EXP id:({})",
                        user_id.get_value()
                    );
                    0
                }
            };
            self.active_viewers_data
                .insert(user_id.clone(), existing_viewer_experience + 1);

            if self.active_viewers_data[&user_id] != TICKS_PER_EXP_POINT {
                return;
            }

            green_ln!("Giving user an exp point! id: {}", user_id.get_value());

            let mut current_stats = UserRpgStats::load_or_default(channel.clone(), user_id.clone());
            current_stats.add_experience_points(1);
            current_stats.save(channel.clone(), user_id.clone());

            self.active_viewers_data.insert(user_id.clone(), 0);
        }
    }
}
