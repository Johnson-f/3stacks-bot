-- This table currently stores stock symbols for the earnings calendar, and more in the future.
CREATE TABLE watchlist_symbols (symbol TEXT PRIMARY KEY);

-- Basic commands to manage the table:

-- Add a symbol to the table
INSERT INTO watchlist_symbols (symbol) VALUES ('AAPL');

-- Remove a symbol from the table
DELETE FROM watchlist_symbols WHERE symbol = 'AAPL';

-- List all symbols in the table
SELECT * FROM watchlist_symbols;

-- Count the number of stocks in the table
SELECT COUNT(*) FROM watchlist_symbols;
