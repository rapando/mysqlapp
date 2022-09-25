create database rust;
use rust;

create table user (
    user_id int primary key auto_increment,
    first_name varchar(20) not null,
    alive tinyint(1) not null,
    salary decimal(9,2) not null
);

insert into user (first_name, alive, salary) values ('Sam', '1', '4354.90');