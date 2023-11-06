CREATE TABLE
  IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    is_completed BOOLEAN DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP
  );

CREATE TRIGGER updated_at_todo
UPDATE OF title,
is_completed ON todo BEGIN
UPDATE todo
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = New.id;

END;

CREATE TRIGGER completed_at_todo
UPDATE OF is_completed ON todo BEGIN
UPDATE todo
SET
  completed_at = CASE
    WHEN New.is_completed = 1 THEN CURRENT_TIMESTAMP
    ELSE NULL
  END
WHERE
  id = New.id;

END;
