CREATE TABLE `Users` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`create_time` datetime COMMENT 'Create Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY (`discord_id`)
);

CREATE TABLE `Messages` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`channel_id` int NOT NULL,
	`user_id` int NOT NULL,
	`emote_id` int NOT NULL,
	`create_time` datetime COMMENT 'Create Time',
	`message_time` datetime COMMENT 'Message Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY `message_idx` (`emote_id`, `message_time`, `discord_id`)
);


CREATE TABLE `Guilds` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`create_time` datetime COMMENT 'Create Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY (`discord_id`)
);

CREATE TABLE `Emotes` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`create_time` datetime COMMENT 'Create Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY (`discord_id`)
);

CREATE TABLE `Channels` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`create_time` datetime COMMENT 'Create Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY (`discord_id`)
);

CREATE TABLE `Sticker_Use` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`channel_id` int NOT NULL,
	`user_id` int NOT NULL,
	`sticker_id` int NOT NULL,
	`create_time` datetime COMMENT 'Create Time',
	`message_time` datetime COMMENT 'Message Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY `sticker_usage_idx` (`sticker_id`, `message_time`, `discord_id`)
);

CREATE TABLE `Stickers` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`create_time` datetime COMMENT 'Create Time',
	`discord_id` bigint unsigned COMMENT 'Discord ID',
	PRIMARY KEY (`id`),
	UNIQUE KEY (`discord_id`)
);

CREATE TABLE `Settings` (
	`id` int NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
	`last_edit` datetime COMMENT 'Last Edit',
	`guild_id` bigint unsigned NOT NULL COMMENT 'Guild ID',
	`setting` JSON NOT NULL COMMENT 'JSON Payload',
	PRIMARY KEY (`id`)
);