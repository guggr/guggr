# Concepts

Before getting started with guggr, let's take a quick look at the core entity concept.

Everything starts with your personal account, a guggr **user**. One or multiple **users** are part of a **group** with their respective **role** in that **group**. A **user** might be a group member, giving them read access, or an admin (or even the group owner) with higher privileges.

**Groups** are essential as **jobs** are bound to them. A **job** might be an **HTTP** or **Ping** job, depending on the type of request that is sent for health checks. If a **job** is run, the respective **job results** are persisted in the DB, waiting to be queried :).
