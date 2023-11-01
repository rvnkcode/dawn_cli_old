CREATE TABLE
  IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    is_completed BOOLEAN DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
  );

-- TODO: Test trigger for multiple updates
CREATE TRIGGER updated_at_todo
UPDATE OF title,
is_completed ON todo BEGIN
UPDATE todo
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = New.id;

END;
