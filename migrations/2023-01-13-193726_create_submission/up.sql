CREATE TABLE submission (
  id SERIAL PRIMARY KEY,
  last_testcase VARCHAR(1000),
  memory REAL,
  memory_display VARCHAR(10),
  memory_percentile REAL,
  notes TEXT,
  runtime REAL,
  runtime_percentile REAL,
  status_code INT,
  timestamp INT
)