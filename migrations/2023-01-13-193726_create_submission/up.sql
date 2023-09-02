CREATE TABLE submission (
  id SERIAL PRIMARY KEY,
  memory REAL,
  memory_display VARCHAR(10),
  memory_percentile REAL,
  notes TEXT,
  runtime REAL,
  runtime_percentile REAL,
  status_code INT,
  timestamp INT
)