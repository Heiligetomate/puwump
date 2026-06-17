 
### Basic Insert
```sql
INSERT INTO exercise (id, name, instructions) VALUES (?1, ?2, ?3)

INSERT INTO ingredient (name, id) VALUES (?1, ?2)
```

### Basic Delete
```sql
DELETE FROM exercise WHERE id = ?1

DELETE FROM ingredient WHERE id = ?1
``` 

### Basic Select
```sql
SELECT id, instructions, name FROM exercise WHERE id = ?1

SELECT name FROM ingredient WHERE id = ?1
```

### Basic Update 
```sql 
UPDATE ingredient_in_meal SET amount_gr = amount_gr + ?2 WHERE id = ?1

UPDATE plan_exercise SET reps = reps + 1 WHERE id = ?
```

### Select ordered, ignore case 
```sql
SELECT id FROM exercise ORDER BY name COLLATE NOCASE ASC

SELECT name FROM ingredient ORDER BY name COLLATE NOCASE ASC
```

### Join 
```sql
SELECT im.id, i.id, i.name, im.amount_gr
     FROM ingredient_in_meal im
     JOIN ingredient i ON i.id = im.ingredient_id
     WHERE im.meal_id = ?1

SELECT pe.id, e.id, e.name, e.instructions, pe.order_index, pe.reps
     FROM plan_exercise pe
     JOIN exercise e ON e.id = pe.exercise_id
     WHERE pe.plan_id = ?1
     ORDER BY pe.order_index ASC
```

### Move order 
```rs 
pub fn move_plan_exercise(&self, id: Uuid, diff: i8) -> Result<()> {
    let (plan_id, current): (String, i16) = self
        .con
        .query_row("SELECT plan_id, order_index FROM plan_exercise WHERE id = ?1", params![id.to_string()], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

    let target = current + diff as i16;
    if target < 0 {
        return Ok(());
    }

    // Park row at -1
    self.con
        .execute("UPDATE plan_exercise SET order_index = -1 WHERE id = ?1", params![id.to_string()])?;

    // Move neighbour to old slot
    let moved = self
        .con
        .execute("UPDATE plan_exercise SET order_index = ?3 WHERE plan_id = ?1 AND order_index = ?2", params![plan_id, target, current])?;
    
    // If doesnt exists, undo the park and return (count affected rows)
    if moved == 0 {
        self.con
            .execute("UPDATE plan_exercise SET order_index = ?2 WHERE id = ?1", params![id.to_string(), current])?;
        return Ok(());
    }

    // Move row to target
    self.con
        .execute("UPDATE plan_exercise SET order_index = ?2 WHERE id = ?1", params![id.to_string(), target])?;

    Ok(())
}
```
