-- Your SQL goes here
create table if not exists user_data(
	user_id bigint generated always as identity PRIMARY KEY,
	user_name text NOT NULL,
	user_alias text NOT NULL,
	user_address text NOT NULL,
	user_create_ts timestamp with time zone NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC'),
	user_update_ts timestamp with time zone NOT NULL DEFAULT (current_timestamp AT TIME ZONE 'UTC')
);