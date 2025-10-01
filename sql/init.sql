create table _group (
                        id varchar(255) not null primary key,
                        create_time timestamp not null,
                        update_time timestamp not null,
                        group_name varchar(255),
                        description varchar(64),
                        logo_object_key varchar(255),
                        nickname varchar,
                        can_start_ai boolean
);

create table _role (
                       id varchar(255) not null primary key,
                       role_name varchar(255),
                       description varchar(255),
                       create_time timestamp not null,
                       update_time timestamp not null
);

create table _user (
                       id varchar(255) not null primary key,
                       password varchar(255),
                       username varchar(255) constraint _user_pk unique,
                       create_time timestamp not null,
                       update_time timestamp not null,
                       nickname varchar(255),
                       group_id varchar(255),
                       role_id varchar(255),
                       description varchar(255)
);

create table role_authority (
                                id varchar(255) not null primary key,
                                create_time timestamp not null,
                                update_time timestamp not null,
                                description varchar(64),
                                role_id varchar(255),
                                authority_id varchar(255)
);

create table white_list (
                            id varchar(255) not null primary key,
                            path varchar(255) not null,
                            create_time timestamp not null,
                            update_time timestamp not null,
                            description varchar(255)
);

create table _authority (
                            id varchar(255) not null primary key,
                            authority varchar(255),
                            name varchar(255),
                            authority_group_id varchar(255),
                            create_time timestamp,
                            update_time timestamp
);

create table authority_group (
                                 id varchar(255) not null primary key,
                                 group_name varchar(255),
                                 description varchar(255),
                                 create_time timestamp,
                                 update_time timestamp
);

create table manage_device_dictionary
(
    id          varchar(255)                              not null
        primary key,
    domain      integer                                   not null,
    device_type integer                                   not null,
    sub_type    integer                                   not null,
    device_name varchar(32) default ''::character varying not null,
    device_desc varchar(100),
    create_time timestamp   default now(),
    update_time timestamp                                 not null
);

create table manage_device (
                               id varchar(255) not null primary key,
                               device_sn varchar(255),
                               device_name varchar(255),
                               nickname varchar(255),
                               group_id varchar(255),
                               device_type integer,
                               sub_type integer,
                               domain integer,
                               firmware_version varchar(255),
                               compatible_status boolean,
                               version varchar(255),
                               device_index varchar(255),
                               child_sn varchar(255),
                               bound_status boolean,
                               login_time bigint,
                               device_desc text,
                               url_normal text,
                               url_select text,
                               longitude double precision,
                               height double precision,
                               latitude double precision,
                               create_time timestamp,
                               update_time timestamp,
                               bound_time timestamp,
                               dongle_4g varchar(255)
);

create table manage_media (
                              id varchar(255) not null primary key,
                              flight_id varchar(255),
                              media_name varchar(255),
                              media_device varchar(255),
                              media_absolute_altitude double precision,
                              media_relative_altitude double precision,
                              media_lat double precision,
                              media_lng double precision,
                              media_object_key varchar(255),
                              media_path text,
                              capture_time timestamp,
                              media_size bigint,
                              create_time timestamp,
                              update_time timestamp,
                              group_id varchar(255)
);

create table task_info (
                           id varchar(255) not null primary key,
                           group_id varchar(255),
                           device_sn varchar(255),
                           cover_object_key varchar(255),
                           task_name varchar(255),
                           rth_altitude integer,
                           task_file_object_key varchar(255),
                           out_of_control_action integer,
                           task_type integer,
                           wayline_type integer,
                           task_file_md5 varchar(255),
                           is_deleted boolean default false,
                           execute_count integer default 0,
                           create_time timestamp,
                           update_time timestamp,
                           task_operation varchar,
                           is_simulated boolean,
                           is_ai boolean
);

create table task_job (
                          id varchar(255) not null primary key,
                          group_id varchar(255),
                          device_sn varchar(255),
                          task_info_id varchar(255),
                          execute_time timestamp,
                          completed_time timestamp,
                          flight_id varchar(255),
                          error_code integer,
                          total_media integer default 0,
                          task_progress integer,
                          uploaded_media_count integer default 0,
                          task_flight_distance integer,
                          create_time timestamp,
                          update_time timestamp,
                          zip_url varchar(255),
                          task_name varchar,
                          task_status varchar,
                          drone_path_object_key varchar,
                          departure_time timestamp,
                          arrival_time timestamp,
                          ai_doc varchar,
                          break_point varchar(255)
);

create table device_hms (
                            id varchar(255) not null primary key,
                            device_sn varchar(255),
                            hms_code varchar(255),
                            group_id varchar(255),
                            hms_level integer,
                            hms_module integer,
                            is_gone boolean,
                            create_time timestamp,
                            update_time timestamp
);

create table _model (
                        id varchar(255) not null primary key,
                        group_id varchar(255),
                        create_time timestamp,
                        update_time timestamp,
                        flight_id varchar,
                        type varchar,
                        dataset_url varchar,
                        visible boolean,
                        process_status integer,
                        task_name varchar,
                        longitude double precision,
                        latitude double precision
);

create table ai_model (
                          id varchar(255) not null primary key,
                          object_key varchar(255),
                          name varchar(255),
                          label varchar(255),
                          md5 varchar(255),
                          create_time timestamp,
                          update_time timestamp,
                          size bigint
);

create table ai_img (
                        id varchar(255) not null primary key,
                        create_time timestamp not null,
                        update_time timestamp not null,
                        flight_id varchar,
                        object_key varchar,
                        longitude double precision,
                        latitude double precision,
                        number integer,
                        timestamp bigint,
                        group_id varchar,
                        label varchar,
                        task_name varchar
);

create table _iframe (
                         id varchar(255) not null primary key,
                         create_time timestamp,
                         update_time timestamp,
                         group_id varchar(255),
                         addr varchar(255)
);

create table _label (
                        id varchar(255) not null primary key,
                        create_time timestamp,
                        update_time timestamp,
                        group_id varchar(255),
                        is_show boolean,
                        labels varchar(255),
                        cover_file varchar(255)
);
