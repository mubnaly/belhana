-- 0014: Household-wide food-preference selection.
-- NULL means "no filter": every recipe remains available. A non-NULL id refers
-- to an editorial diet plan id from `guidance::guide().diets`. This is a user
-- preference, never a medical protocol.
ALTER TABLE households ADD COLUMN selected_diet_id TEXT;
