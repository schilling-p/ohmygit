CREATE FUNCTION update_updated_at_column() RETURNS trigger
    LANGUAGE plpgsql
AS $$
BEGIN
    NEW.updated_at = now();
RETURN NEW;
END;
$$;

CREATE TABLE users (
                       id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                       username text DEFAULT 'username'::text NOT NULL,
                       email text DEFAULT 'users email'::text NOT NULL,
                       hashed_pw text DEFAULT 'users hashed password'::text NOT NULL,
                       created_at timestamp with time zone DEFAULT now() NOT NULL,
                       updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE organizations (
                               id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                               name text DEFAULT 'organizations name'::text NOT NULL,
                               description text DEFAULT 'organizations description'::text NOT NULL,
                               created_at timestamp with time zone DEFAULT now() NOT NULL,
                               updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON organizations FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE repositories (
                              id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                              owner_id uuid REFERENCES users(id),
                              owner_org_id uuid REFERENCES organizations(id),
                              name text DEFAULT 'repository name'::text NOT NULL,
                              is_public boolean DEFAULT true NOT NULL,
                              description text DEFAULT 'repository description':: text,
                              created_at timestamp with time zone DEFAULT now() NOT NULL,
                              updated_at timestamp with time zone DEFAULT now() NOT NULL,
                              CONSTRAINT check_owner_or_org CHECK ((((owner_id IS NOT NULL) AND (owner_org_id IS NULL)) OR ((owner_id IS NULL) AND (owner_org_id IS NOT NULL))))
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON repositories FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE branches (
                          id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                          creator_id uuid NOT NULL REFERENCES users(id)
                            ON DELETE CASCADE
                            ON UPDATE CASCADE,
                          repository_id uuid NOT NULL REFERENCES repositories(id)
                            ON DELETE CASCADE
                            ON UPDATE CASCADE,
                          name text DEFAULT 'branch name'::text NOT NULL,
                          created_at timestamp with time zone DEFAULT now() NOT NULL,
                          updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON branches FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE issues (
                        id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                        creator_id uuid NOT NULL REFERENCES users(id)
                            ON UPDATE CASCADE
                            ON DELETE CASCADE,
                        repository_id uuid NOT NULL REFERENCES repositories(id)
                            ON UPDATE CASCADE
                            ON DELETE CASCADE,
                        title text DEFAULT 'issue title'::text NOT NULL,
                        body text DEFAULT 'issue body'::text NOT NULL,
                        status text DEFAULT 'issue status'::text NOT NULL,
                        created_at timestamp with time zone DEFAULT now() NOT NULL,
                        updated_at timestamp with time zone DEFAULT now() NOT NULL,
                        CONSTRAINT status CHECK ((status = ANY (ARRAY['open'::text, 'closed'::text, 'progress'::text])))
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON issues FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE organizations_members (
                                PRIMARY KEY (user_id, organization_id),
                                user_id uuid NOT NULL REFERENCES users(id)
                                    ON UPDATE CASCADE
                                    ON DELETE CASCADE,
                                organization_id uuid NOT NULL REFERENCES organizations(id)
                                    ON UPDATE CASCADE
                                    ON DELETE CASCADE,
                                role text DEFAULT 'members role in the organization'::text NOT NULL,
                                created_at timestamp with time zone DEFAULT now() NOT NULL,
                                updated_at timestamp with time zone DEFAULT now() NOT NULL,
                                CONSTRAINT check_role CHECK ((role = ANY (ARRAY['owner'::text, 'admin'::text, 'maintainer'::text, 'developer'::text, 'guest'::text])))
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON organizations_members FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE user_repository_roles (
                              PRIMARY KEY (user_id, repository_id),
                              user_id uuid NOT NULL REFERENCES users(id)
                                  ON UPDATE CASCADE
                                  ON DELETE CASCADE,
                              repository_id uuid NOT NULL REFERENCES repositories(id)
                                  ON UPDATE CASCADE
                                  ON DELETE CASCADE,
                              role text DEFAULT 'users role in the repo'::text NOT NULL,
                              created_at timestamp with time zone DEFAULT now() NOT NULL,
                              updated_at timestamp with time zone DEFAULT now() NOT NULL,
                              CONSTRAINT user_repository_roles_role_check CHECK (role = ANY (ARRAY ['owner'::text, 'developer'::text, 'maintainer'::text, 'guest'::text]))
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON user_repository_roles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE comments (
                          id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                          creator_id uuid NOT NULL REFERENCES users(id)
                              ON DELETE CASCADE
                              ON UPDATE CASCADE,
                          target_id uuid NOT NULL,
                          target_type text DEFAULT 'comment target type'::text NOT NULL,
                          body text DEFAULT 'comment body'::text NOT NULL,
                          created_at timestamp with time zone DEFAULT now() NOT NULL,
                          updated_at timestamp with time zone DEFAULT now() NOT NULL,
                          CONSTRAINT check_target_type CHECK ((target_type = ANY (ARRAY['issue'::text, 'merge_request'::text])))
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON comments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE merge_requests (
                                id uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
                                creator_id uuid NOT NULL REFERENCES users(id)
                                    ON UPDATE CASCADE
                                    ON DELETE CASCADE,
                                repository_id uuid NOT NULL REFERENCES repositories(id)
                                    ON UPDATE CASCADE
                                    ON DELETE CASCADE,
                                source_branch_id uuid NOT NULL REFERENCES branches(id)
                                    ON UPDATE CASCADE
                                    ON DELETE CASCADE,
                                target_branch_id uuid NOT NULL REFERENCES branches(id)
                                    ON UPDATE CASCADE
                                    ON DELETE CASCADE,
                                title text DEFAULT 'merge requests title'::text NOT NULL,
                                description text DEFAULT 'merge request description'::text NOT NULL,
                                status text DEFAULT 'merge request status text'::text NOT NULL,
                                created_at timestamp with time zone DEFAULT now(),
                                updated_at timestamp with time zone DEFAULT now() NOT NULL,
                                CONSTRAINT merge_requests_status_check CHECK ((status = ANY (ARRAY['open'::text, 'closed'::text, 'merged'::text])))
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON merge_requests FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();