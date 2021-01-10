CREATE OR REPLACE FUNCTION delete_old_users() RETURNS trigger
    LANGUAGE plpgsql
    AS $func1$
BEGIN
    delete from user_room where users_id in (select users.id from users  where users.updated_at < NOW() - INTERVAL '2 days');
    DELETE FROM users WHERE updated_at < NOW() - INTERVAL '2 days';
  RETURN NULL;
END;
$func1$;;
CREATE TRIGGER clear_users AFTER insert ON users
EXECUTE PROCEDURE delete_old_users();


CREATE OR REPLACE FUNCTION delete_unused_rooms() RETURNS TRIGGER 
  LANGUAGE plpgsql
  AS $func$
   BEGIN
      delete from room where room.id not in (select room_id from user_room);
      RETURN NEW;
   END;
$func$;;
CREATE TRIGGER clear_rooms AFTER delete ON user_room
EXECUTE PROCEDURE delete_unused_rooms();