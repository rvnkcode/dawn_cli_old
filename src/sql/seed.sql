INSERT INTO
  todo (title, note, is_completed, completed_at, is_deleted)
VALUES
  ("This is the sample To-Do", "Note test", 0, NULL, 0),
  ("Completed To-Do sample", "Note test", 1, CURRENT_TIMESTAMP, 0),
  ("Deleted", "Note test", 0, NULL, 1),
  ("Deleted and completed", NULL, 1, CURRENT_TIMESTAMP, 1);
