### Setting up Postgres in Ubuntu 20.04 WSL2

Site: https://chloesun.medium.com/set-up-postgresql-on-wsl2-and-connect-to-postgresql-with-pgadmin-on-windows-ca7f0b7f38ab

Install PostgreSQL on WSL 2
Here comes the WSL 2, much easier, and you can visit the Windows WSL documentation for more detail.

1. Open your WSL terminal (ie. Ubuntu 18.04).
2. Update your Ubuntu packages: sudo apt update
3. Once the packages have updated, install PostgreSQL (and the -contrib package which has some helpful utilities) with: 

```bash
sudo apt install postgresql postgresql-contrib
```

4. Confirm the installation and get the version number: 

```bash
psql --version
```

Confirm the installation and get the version number
There are 3 commands you need to know once PostgreSQL is installed:

- "sudo service postgresql status" for checking the status of your database.
- "sudo service postgresql start" to start running your database.
- "sudo service postgresql stop" to stop running your database.

The default admin user, "postgres", needs a password assigned in order to connect to a database. To set a password:

1. Enter the command: ```sudo passwd postgres```
2. You will get a prompt to enter your new password.
3. Close and reopen your terminal.

**If you have login problems, check the link (here)[https://stackoverflow.com/questions/2942485/psql-fatal-ident-authentication-failed-for-user-postgres]**

### Setting up Diesel DB

Used the diesel get started docs (here)[https://diesel.rs/guides/getting-started]