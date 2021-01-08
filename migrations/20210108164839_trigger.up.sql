CREATE FUNCTION delete_old_rows() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
  DELETE FROM users WHERE updated_at < NOW() - INTERVAL '2 days';
  RETURN NULL;
END;
$$;