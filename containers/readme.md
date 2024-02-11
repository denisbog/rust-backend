### setup ec2 instance

yum install mariadb105-server -y
sudo systemctl enable mariadb
sudo systemctl start mariadb

### configure db

sudo mysql_secure_installation
sudo mysql -u root

### create custom db

create database items;

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
Environment="DATABASE_URL=mariadb://admin:admin@localhost/items" "CLIENT_ID=2967739669909794" "CLIENT_SECRET=3f5afe538ee1780803b60306ef89582f"
ExecStart=/app/app
[Install]
WantedBy=multi-user.target
