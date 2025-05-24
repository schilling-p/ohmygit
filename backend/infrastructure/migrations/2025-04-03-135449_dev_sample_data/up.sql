--
-- Data for Name: organizations; Type: TABLE DATA; Owner: admin
--

INSERT INTO organizations (id, name, description, created_at, updated_at) VALUES ('ffd6b166-6694-4a23-928a-e2d1869b6a17', 'code university', 'code university of applied sciences berlin, we love to educate you', '2025-04-02 11:55:11.575+00', '2025-04-02 11:55:15.197+00');
INSERT INTO organizations (id, name, description, created_at, updated_at) VALUES ('6ee92e9d-466c-40a4-ab06-15a8d1c03fd9', 'linux foundation', 'the linux foundation aids in development of open source software', '2025-04-02 09:55:57.623463+00', '2025-04-02 09:55:57.623463+00');
INSERT INTO organizations (id, name, description, created_at, updated_at) VALUES ('30a2383b-3e8b-4aff-97e5-e260b890ba4a', 'pauls gang', 'we develop software bro', '2025-04-02 09:55:57.623463+00', '2025-04-02 09:55:57.623463+00');


--
-- Data for Name: users; Type: TABLE DATA; Owner: admin
--

INSERT INTO users (id, username, email, hashed_pw, created_at, updated_at) VALUES ('c8bf561e-5d0e-4baa-ad59-734debf37bb3', 'paul', 'paul.schilling@code.berlin', '$argon2id$v=19$m=19456,t=2,p=1$3P80ul+evy7MfQ9jDCrAPw$ubpQA6uQx+j7l3FBFuodeeXnf9fQPCqNy7WjlrhXp1k', '2025-04-02 11:51:17.923+00', '2025-04-02 11:51:21.191+00');
INSERT INTO users (id, username, email, hashed_pw, created_at, updated_at) VALUES ('7d6e9871-639b-472a-b25b-df6b10178f71', 'matilda', 'matilda.junghanns@gmx.net', 'paulistderbeste', '2025-04-02 11:51:57.59+00', '2025-04-02 11:52:00.56+00');
INSERT INTO users (id, username, email, hashed_pw, created_at, updated_at) VALUES ('81d7d778-c455-49a5-abe1-3703deb51ed4', 'rupert', 'rupert.junghanns@gmx.de', 'mamaistdiebeste', '2025-04-02 11:52:19.303+00', '2025-04-02 11:52:20.276+00');
INSERT INTO users (id, username, email, hashed_pw, created_at, updated_at) VALUES ('19530df9-661d-4377-956b-a64cd0fbf18d', 'thomas', 'thomas.frey@code.berlin', 'praisejah', '2025-04-02 11:52:52.532+00', '2025-04-02 11:52:53.312+00');
INSERT INTO users (id, username, email, hashed_pw, created_at, updated_at) VALUES ('5281a643-a2ab-4c4d-a354-9bd93e707262', 'wessel', 'wessel.weernink@code.berlin', 'iloveswitzerland', '2025-04-02 11:53:19.141+00', '2025-04-02 11:53:19.952+00');


--
-- Data for Name: repositories; Type: TABLE DATA; Owner: admin
--

INSERT INTO repositories (id, owner_id, owner_org_id, name, is_public, created_at, updated_at) VALUES ('58b187aa-1206-4777-ae99-9e3d2cd4ad8e', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', NULL, 'rust-learning', false, '2025-04-02 10:11:00.572093+00', '2025-04-02 10:11:00.572093+00');
INSERT INTO repositories (id, owner_id, owner_org_id, name, is_public, created_at, updated_at) VALUES ('c306c37c-892e-4761-8af2-073b26374ca5', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', NULL, 'ohmygit', true, '2025-04-02 10:15:09.717645+00', '2025-04-02 10:15:09.717645+00');
INSERT INTO repositories (id, owner_id, owner_org_id, name, is_public, created_at, updated_at) VALUES ('6cfc63fd-7140-40d4-84c8-a686c43aa159', '7d6e9871-639b-472a-b25b-df6b10178f71', NULL, 'beauty_salon', true, '2025-04-02 10:15:09.717645+00', '2025-04-02 10:15:09.717645+00');
INSERT INTO repositories (id, owner_id, owner_org_id, name, is_public, created_at, updated_at) VALUES ('f7c33199-ea61-420d-b66c-7256e3b1e89e', '19530df9-661d-4377-956b-a64cd0fbf18d', NULL, 'thomas_website', false, '2025-04-02 10:29:07.008085+00', '2025-04-02 10:29:07.008085+00');
INSERT INTO repositories (id, owner_id, owner_org_id, name, is_public, created_at, updated_at) VALUES ('9cc6774c-b5d6-485e-892d-b862318596a8', '81d7d778-c455-49a5-abe1-3703deb51ed4', NULL, 'portfolio manager', true, '2025-04-02 12:45:21.783157+00', '2025-04-02 12:45:21.783157+00');
INSERT INTO repositories (id, owner_id, owner_org_id, name, is_public, created_at, updated_at) VALUES ('2d0b5088-4662-4890-9695-9cd1f66702a4', NULL, '6ee92e9d-466c-40a4-ab06-15a8d1c03fd9', 'linux kernel', true, '2025-04-02 12:45:57.119733+00', '2025-04-02 12:45:57.119733+00');


--
-- Data for Name: branches; Type: TABLE DATA; Owner: admin
--

INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('7378b52f-2c9b-499f-a2d9-8a90ade47a87', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '58b187aa-1206-4777-ae99-9e3d2cd4ad8e', 'main', '2025-04-02 12:32:21.642064+00', '2025-04-02 12:32:21.642064+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('137cc638-85d3-426e-9330-01c4b1e02be0', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '58b187aa-1206-4777-ae99-9e3d2cd4ad8e', 'development', '2025-04-02 12:32:21.642064+00', '2025-04-02 12:32:21.642064+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('3e42859e-b962-4266-9924-5261b53793a2', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '58b187aa-1206-4777-ae99-9e3d2cd4ad8e', 'feature', '2025-04-02 12:32:21.642064+00', '2025-04-02 12:32:21.642064+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('d79bc309-9480-4620-b412-cd88515abe6d', '7d6e9871-639b-472a-b25b-df6b10178f71', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'master', '2025-04-02 12:33:07.989601+00', '2025-04-02 12:33:07.989601+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('1b94dd75-5fe6-4fda-ba90-bb9adb614aa3', '7d6e9871-639b-472a-b25b-df6b10178f71', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'development', '2025-04-02 12:33:07.989601+00', '2025-04-02 12:33:07.989601+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('d7260b69-100c-4a80-badc-83d9fefe0079', '7d6e9871-639b-472a-b25b-df6b10178f71', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'test', '2025-04-02 12:33:07.989601+00', '2025-04-02 12:33:07.989601+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('fc5af26f-b47c-48ab-9ff9-998d5f89aeb5', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', 'c306c37c-892e-4761-8af2-073b26374ca5', 'main', '2025-04-02 12:38:34.710544+00', '2025-04-02 12:38:34.710544+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('7f8fc400-341c-428d-9a0a-6c60eb7875a9', '5281a643-a2ab-4c4d-a354-9bd93e707262', 'c306c37c-892e-4761-8af2-073b26374ca5', 'development', '2025-04-02 12:38:34.710544+00', '2025-04-02 12:38:34.710544+00');
INSERT INTO branches (id, creator_id, repository_id, name, created_at, updated_at) VALUES ('25e25813-192b-44d1-8080-b7b7110ae7af', '81d7d778-c455-49a5-abe1-3703deb51ed4', 'c306c37c-892e-4761-8af2-073b26374ca5', 'feature', '2025-04-02 12:38:34.710544+00', '2025-04-02 12:38:34.710544+00');


--
-- Data for Name: issues; Type: TABLE DATA; Owner: admin
--

INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('6102120a-fba6-4f4a-a570-4e8a195c0e97', '7d6e9871-639b-472a-b25b-df6b10178f71', 'c306c37c-892e-4761-8af2-073b26374ca5', 'missing logout button', 'the app is missing a button to log out', 'open', '2025-04-02 12:42:57.483907+00', '2025-04-02 12:42:57.483907+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('c065e2f9-0efb-4995-9596-1d6a257739ea', '5281a643-a2ab-4c4d-a354-9bd93e707262', 'c306c37c-892e-4761-8af2-073b26374ca5', 'dashboard missing', 'since there is no dashboard I cannot really use the app', 'open', '2025-04-02 12:42:57.483907+00', '2025-04-02 12:42:57.483907+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('211b0874-f642-4041-ae32-e744c9543a7e', '19530df9-661d-4377-956b-a64cd0fbf18d', 'c306c37c-892e-4761-8af2-073b26374ca5', 'feature x ', 'this feature, x, would really help me using the application', 'open', '2025-04-02 12:42:57.483907+00', '2025-04-02 12:42:57.483907+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('ea5cb99b-ee90-472f-be05-0d48e5c9f9cc', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', 'c306c37c-892e-4761-8af2-073b26374ca5', 'setup', 'install all the software to develop the this feature, x, would really help me using the application', 'closed', '2025-04-02 12:42:57.483907+00', '2025-04-02 12:42:57.483907+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('9e2f1953-4ee1-469d-b94c-f797639b4415', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'bug in login page', 'whenever I do this, the page crashes', 'open', '2025-04-02 12:44:49.415976+00', '2025-04-02 12:44:49.415976+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('03985b2b-ec4f-4da5-97d3-7f045cefa622', '5281a643-a2ab-4c4d-a354-9bd93e707262', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'bug in landing page', 'after login, if I click this, the browser freezes', 'open', '2025-04-02 12:44:49.415976+00', '2025-04-02 12:44:49.415976+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('2341eeb5-9bb7-483c-9b8c-2f753d68eace', '81d7d778-c455-49a5-abe1-3703deb51ed4', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'wrong permission', 'when I start the app, my calculator opens', 'closed', '2025-04-02 12:44:49.415976+00', '2025-04-02 12:44:49.415976+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('47d70b9a-1162-49c1-a3fe-ccd9b25f5630', '19530df9-661d-4377-956b-a64cd0fbf18d', '9cc6774c-b5d6-485e-892d-b862318596a8', 'lost all money', 'due to a bug in the software I lost all my money, please help', 'open', '2025-04-02 12:47:48.900818+00', '2025-04-02 12:47:48.900818+00');
INSERT INTO issues (id, creator_id, repository_id, title, body, status, created_at, updated_at) VALUES ('96ddace9-1883-4042-91e6-02fdac224242', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '9cc6774c-b5d6-485e-892d-b862318596a8', 'add N26', 'add N26 to the available banks so I can put my money in there as well', 'closed', '2025-04-02 12:47:48.900818+00', '2025-04-02 12:47:48.900818+00');


--
-- Data for Name: merge_requests; Type: TABLE DATA; Owner: admin
--

INSERT INTO merge_requests (id, creator_id, repository_id, source_branch_id, target_branch_id, title, description, created_at, updated_at, status) VALUES ('52ac77e3-ae7c-451d-82a4-26d980bbcf75', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '58b187aa-1206-4777-ae99-9e3d2cd4ad8e', '3e42859e-b962-4266-9924-5261b53793a2', '137cc638-85d3-426e-9330-01c4b1e02be0', 'Ticket-54', 'Ticket-54 feature X', '2025-04-02 12:52:35.180163+00', '2025-04-02 12:52:35.180163+00', 'open');
INSERT INTO merge_requests (id, creator_id, repository_id, source_branch_id, target_branch_id, title, description, created_at, updated_at, status) VALUES ('eb9f270c-5298-44b3-94d4-3799c6640bf9', '7d6e9871-639b-472a-b25b-df6b10178f71', '6cfc63fd-7140-40d4-84c8-a686c43aa159', 'd7260b69-100c-4a80-badc-83d9fefe0079', '1b94dd75-5fe6-4fda-ba90-bb9adb614aa3', 'Ticket-2', 'Ticket-2 feature Y', '2025-04-02 12:52:35.180163+00', '2025-04-02 12:52:35.180163+00', 'open');
INSERT INTO merge_requests (id, creator_id, repository_id, source_branch_id, target_branch_id, title, description, created_at, updated_at, status) VALUES ('10e61bbb-f571-4961-b044-b351b0e8d98d', '5281a643-a2ab-4c4d-a354-9bd93e707262', 'c306c37c-892e-4761-8af2-073b26374ca5', '25e25813-192b-44d1-8080-b7b7110ae7af', '7f8fc400-341c-428d-9a0a-6c60eb7875a9', 'Ticket-53', 'Ticket-53 fix Bug X', '2025-04-02 12:58:53.114882+00', '2025-04-02 12:58:53.114882+00', 'closed');


--
-- Data for Name: comments; Type: TABLE DATA; Owner: admin
--

INSERT INTO comments (id, creator_id, target_id, target_type, body, created_at, updated_at) VALUES ('f88e3a5e-0a3f-4840-99f1-5657b6997fa4', '7d6e9871-639b-472a-b25b-df6b10178f71', '9e2f1953-4ee1-469d-b94c-f797639b4415', 'issue', 'that has to be this way, that is a skill issue on your side', '2025-04-02 13:01:22.829705+00', '2025-04-02 13:02:24.797878+00');
INSERT INTO comments (id, creator_id, target_id, target_type, body, created_at, updated_at) VALUES ('7f6d0ba6-09b8-475a-9d44-3c8bcd6e88d3', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', 'c065e2f9-0efb-4995-9596-1d6a257739ea', 'issue', 'please fix now', '2025-04-02 13:02:24.797878+00', '2025-04-02 13:02:24.797878+00');
INSERT INTO comments (id, creator_id, target_id, target_type, body, created_at, updated_at) VALUES ('f7ad67e0-2771-4adb-82e1-a49a0380c674', '5281a643-a2ab-4c4d-a354-9bd93e707262', '52ac77e3-ae7c-451d-82a4-26d980bbcf75', 'merge_request', 'you could write that code cleaner', '2025-04-02 13:03:34.178829+00', '2025-04-02 13:03:34.178829+00');
INSERT INTO comments (id, creator_id, target_id, target_type, body, created_at, updated_at) VALUES ('3bafb4cc-d15e-4fff-bc32-74e28166f407', 'c8bf561e-5d0e-4baa-ad59-734debf37bb3', '52ac77e3-ae7c-451d-82a4-26d980bbcf75', 'merge_request', 'please explain to me how I could do that ', '2025-04-02 13:04:17.87254+00', '2025-04-02 13:04:17.87254+00');


--
-- Data for Name: organizations_members; Type: TABLE DATA; Owner: admin
--

INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('c8bf561e-5d0e-4baa-ad59-734debf37bb3', '30a2383b-3e8b-4aff-97e5-e260b890ba4a', 'admin', '2025-04-02 09:59:36.635401+00', '2025-04-02 09:59:36.635401+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('7d6e9871-639b-472a-b25b-df6b10178f71', '30a2383b-3e8b-4aff-97e5-e260b890ba4a', 'guest', '2025-04-02 09:59:36.635401+00', '2025-04-02 09:59:36.635401+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('5281a643-a2ab-4c4d-a354-9bd93e707262', '30a2383b-3e8b-4aff-97e5-e260b890ba4a', 'developer', '2025-04-02 09:59:36.635401+00', '2025-04-02 09:59:36.635401+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('19530df9-661d-4377-956b-a64cd0fbf18d', '6ee92e9d-466c-40a4-ab06-15a8d1c03fd9', 'maintainer', '2025-04-02 10:00:57.600349+00', '2025-04-02 10:00:57.600349+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('19530df9-661d-4377-956b-a64cd0fbf18d', 'ffd6b166-6694-4a23-928a-e2d1869b6a17', 'admin', '2025-04-02 10:00:57.600349+00', '2025-04-02 10:00:57.600349+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('5281a643-a2ab-4c4d-a354-9bd93e707262', 'ffd6b166-6694-4a23-928a-e2d1869b6a17', 'owner', '2025-04-02 10:00:57.600349+00', '2025-04-02 10:00:57.600349+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('81d7d778-c455-49a5-abe1-3703deb51ed4', '6ee92e9d-466c-40a4-ab06-15a8d1c03fd9', 'owner', '2025-04-02 10:01:34.386353+00', '2025-04-02 10:01:34.386353+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('7d6e9871-639b-472a-b25b-df6b10178f71', '6ee92e9d-466c-40a4-ab06-15a8d1c03fd9', 'guest', '2025-04-02 10:01:34.386353+00', '2025-04-02 10:01:34.386353+00');
INSERT INTO organizations_members (user_id, organization_id, role, created_at, updated_at) VALUES ('5281a643-a2ab-4c4d-a354-9bd93e707262', '6ee92e9d-466c-40a4-ab06-15a8d1c03fd9', 'developer', '2025-04-02 10:01:54.598738+00', '2025-04-02 10:01:54.598738+00');


--
-- Data for Name: user_repository_roles; Type: TABLE DATA; Owner: admin
--

INSERT INTO user_repository_roles (user_id, repository_id, role, created_at, updated_at) VALUES ('c8bf561e-5d0e-4baa-ad59-734debf37bb3', 'c306c37c-892e-4761-8af2-073b26374ca5', 'owner', '2025-04-02 12:28:37.64902+00', '2025-04-02 12:28:37.64902+00');
INSERT INTO user_repository_roles (user_id, repository_id, role, created_at, updated_at) VALUES ('7d6e9871-639b-472a-b25b-df6b10178f71', 'c306c37c-892e-4761-8af2-073b26374ca5', 'guest', '2025-04-02 12:28:37.64902+00', '2025-04-02 12:28:37.64902+00');
INSERT INTO user_repository_roles (user_id, repository_id, role, created_at, updated_at) VALUES ('5281a643-a2ab-4c4d-a354-9bd93e707262', 'c306c37c-892e-4761-8af2-073b26374ca5', 'developer', '2025-04-02 12:28:37.64902+00', '2025-04-02 12:28:37.64902+00');
INSERT INTO user_repository_roles (user_id, repository_id, role, created_at, updated_at) VALUES ('5281a643-a2ab-4c4d-a354-9bd93e707262', '58b187aa-1206-4777-ae99-9e3d2cd4ad8e', 'maintainer', '2025-04-02 12:28:37.64902+00', '2025-04-02 12:28:37.64902+00');
INSERT INTO user_repository_roles (user_id, repository_id, role, created_at, updated_at) VALUES ('19530df9-661d-4377-956b-a64cd0fbf18d', 'f7c33199-ea61-420d-b66c-7256e3b1e89e', 'owner', '2025-04-02 12:29:59.268427+00', '2025-04-02 12:29:59.268427+00');
INSERT INTO user_repository_roles (user_id, repository_id, role, created_at, updated_at) VALUES ('c8bf561e-5d0e-4baa-ad59-734debf37bb3', 'f7c33199-ea61-420d-b66c-7256e3b1e89e', 'developer', '2025-04-02 12:29:59.268427+00', '2025-04-02 12:29:59.268427+00');