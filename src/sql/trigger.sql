CREATE TRIGGER updated_at_todo
UPDATE OF title,
is_completed ON todo BEGIN
UPDATE todo
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = New.id;

END;
