create table _user (
                       id varchar(255) not null primary key,
                       password varchar(255),
                       username varchar(255) unique,
                       create_time timestamp not null,
                       update_time timestamp not null,
                       nickname varchar(255),
                       group_id varchar(255),
                       role_id varchar(255),
                       description varchar(255)
);

-- 角色表
create table _role (
                       id varchar(255) not null primary key,
                       name varchar(20),
                       authority_role_id varchar(255),
                       create_time timestamp not null,
                       update_time timestamp not null
);

-- 权限表
create table _authority (
                            id varchar(255) not null primary key,
                            authority varchar(255), -- 权限路径
                            create_time timestamp not null,
                            update_time timestamp not null
);

-- 角色权限关系表
create table _authority_role (
                                 id varchar(255) not null primary key,
                                 role_id varchar(255),
                                 authority_id varchar(255),
                                 create_time timestamp not null,
                                 update_time timestamp not null
);

-- 用户组表
create table _group (
                        id varchar(255) not null primary key,
                        name varchar(255),
                        create_time timestamp not null,
                        update_time timestamp not null
);

-- 用户权限覆盖表
create table _user_authority_override (
                                          id varchar(255) not null primary key,
                                          user_id varchar(255) not null,
                                          authority_id varchar(255) not null,
                                          allow boolean,
                                          create_time timestamp not null,
                                          update_time timestamp not null
);
