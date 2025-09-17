import sqlite3


def demonstrate_rollback_default():
    print("--- Demonstrating Implicit ROLLBACK ---")

    # 1. Connect to an in-memory SQLite database
    # This creates a new, empty database each time
    conn = sqlite3.connect(":memory:")
    cursor = conn.cursor()

    # 2. Create a table
    cursor.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
    print("Table 'users' created.")

    # 3. Insert a row (this implicitly starts a transaction)
    print("Attempting to insert 'Alice'...")
    cursor.execute("INSERT INTO users (name) VALUES (?)", ("Alice",))
    print("'Alice' inserted into current transaction.")

    # 4. Check if Alice is visible within the *same* connection (she is)
    cursor.execute("SELECT * FROM users")
    print(f"Users in current transaction (before close): {cursor.fetchall()}")

    # 5. Close the connection without calling commit()
    # This will trigger an implicit ROLLBACK due to the DBAPI default.
    print("Closing connection without calling commit()...")
    conn.close()
    print("Connection closed.")

    # 6. Try to connect again and retrieve data
    # Since it's an in-memory DB, we'll create a new one, but even if it were file-based,
    # Alice wouldn't be there.
    print("\nReconnecting to check if 'Alice' was saved...")
    # conn_check = sqlite3.connect(":memory:")  # Would be empty anyway
    # cursor_check = conn_check.cursor()  # Not needed for this demo
