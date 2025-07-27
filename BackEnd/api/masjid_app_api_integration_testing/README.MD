# MasjidApp API Integration Testing Library
This library is used by both the public/admin-manager APIs for testing features used by both of them.

## Adding Integration Tests
When adding integration tests for new features, it must be added to in the features directory of the parent folder of this library.

For example, events would be like this

``tests/features/events``

When testing a feature, it must have a file of each testable component: MySQL, Redis, and API endpoints.
The files of each component should be the following:
- Redis -> redis.rs
- MySQL -> my_sql.rs
- API endpoints -> axum_endpoints.rs