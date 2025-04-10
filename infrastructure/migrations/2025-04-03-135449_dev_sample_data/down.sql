-- This file should undo anything in `up.sql`
delete from user_repository_roles;
delete from organizations_members;
delete from comments;
delete from merge_requests;
delete from issues;
delete from branches;
delete from repositories;
delete from users;