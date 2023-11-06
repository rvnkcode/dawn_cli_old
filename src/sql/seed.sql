INSERT INTO
  todo (title, is_completed, completed_at, is_deleted)
VALUES
  ("This is the sample To-Do", 0, NULL, 0),
  ("Completed To-Do sample", 1, CURRENT_TIMESTAMP, 0),
  ("Deleted", 0, NULL, 1);
