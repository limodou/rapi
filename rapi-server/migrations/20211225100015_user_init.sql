-- Add migration script here
CREATE TABLE `user` (
  `id` int(11) NOT NULL AUTO_INCREMENT COMMENT '主键ID',
  `username` varchar(50) NOT NULL COMMENT '用户名',
  `realname` varchar(50) NOT NULL COMMENT '真实姓名',
  `password` varchar(128) DEFAULT '' COMMENT '用户密码(老系统为必填，兼容单点登录修改为可为空。由程序员检验必填)',
  `access_token` varchar(255) DEFAULT '' COMMENT 'token',
  `is_super` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否是管理员：1是0否',
  `error_times` int(2) DEFAULT '1' COMMENT '密码出错次数',
  `freeze_time` datetime DEFAULT NULL COMMENT '冻结时间',
  `is_available` tinyint(1) NOT NULL DEFAULT '1' COMMENT '启用标记:\r\n0禁用\r\n1启用\r\n',
  `is_first_time_login` tinyint(1) DEFAULT '0' COMMENT '是否为第一次登录:\r\n0:是\r\n1:不是',
  `last_landing_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '最近登录时间',
  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  `version` int(11) NOT NULL DEFAULT '1' COMMENT '数据版本',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE KEY `IDX_user_username` (`username`) USING BTREE
) COMMENT='用户表';