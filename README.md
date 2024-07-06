# Final Year Project
My doom

## Instructions for Various Segments of the Project

### Frontend
1. Navigate to the frontend directory:
   ```sh
   cd frontend
   ```
2. Start the frontend server:
   ```sh
   npm start
   ```

### Backend

### Database
1. Start or stop the database:
   ```sh
   pg_ctl start
   pg_ctl stop
   ```
   Or register it as a service by running:
   ```sh
   pg_ctl register -N PostgreSQL
   ```
   from an elevated shell.

2. Default superuser login:
   - Username: postgres
   - Password: redacted

3. Backup the database:
   ```sh
   pg_dump -U postgres -W -F c -b -v -f db_backup.dump social_media_db
   ```

### Resources
- [Dataset for fake and real news](https://www.kaggle.com/datasets/clmentbisaillon/fake-and-real-news-dataset)