### setup ec2 instance

sudo yum install mariadb105-server -y
sudo systemctl enable mariadb
sudo systemctl start mariadb


### create application folder

sudo mkdir /app
sudo chown ec2-user:ec2-user /app

### configure db

sudo mysql_secure_installation
sudo mysql -u root

### create custom db

CREATE DATABASE items CHARACTER SET = 'utf8' COLLATE = 'utf8_general_ci';
-- SELECT * FROM INFORMATION_SCHEMA.SCHEMATA;

grant all on items.* to admin@localhost identified by 'admin';
flush privileges;


### custom environment
export DATABASE_URL=mariadb://admin:admin@localhost/items
export CLIENT_ID=2967739669909794
export CLIENT_SECRET=3f5afe538ee1780803b60306ef89582f

### copy binaries


### allow privileged ports

sudo setcap 'cap_net_bind_service=+ep' /app/app



### edit service

sudo systemctl edit app --force --full

cat /etc/systemd/system/app.service

[Unit]
Description=app daemon
Requires=mariadb.service
[Service]
Type=simple
User=ec2-user
Environment="DATABASE_URL=mariadb://admin:admin@localhost/items" "CLIENT_ID=739986687294200" "CLIENT_SECRET=07edcfd485f8d210c423634c53f038b9" "REDIRECT_URL=https://iadelamine.softfronts.com/api/authorized"
ExecStart=/app/app
[Install]
WantedBy=multi-user.target
