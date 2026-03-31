# User

<p align="center">
	<img src="../assets/logo.svg" width="200" /><br/>
	<b>guggr</b><br/>
	<i>Monitoring that survives your outages.</i>
</p>

## Features

To use guggr, an active account is necessary. Create one by clicking on "Register" or sign in to your account using the "Login" button instead. Once authenticated, your username is shown in the navigation bar, and by clicking on it, a dropdown opens, giving the option to log out or navigate to the account page or the groups overview.

### Account

The account page (`/account`) gives an overview of the currently signed-in account. At the current state, updates to the account or changing the password are not yet possible.

Change the application's theme if you prefer a different 🎨. Your theme setting is persisted in local storage.

### Groups

View and manage your groups on the groups page (`/groups`). Use the "Create new" button to create a new group and modify it in the UI. To add a member to your group, get their user ID and paste it into the "Add Member" field. To edit a group, you need to be the owner or admin of that group.

Groups are necessary to link monitoring jobs to them, so be sure to create one first!

### Jobs

Jobs are managed through the jobs page (`/jobs`). It lists all jobs and gives the option to filter for offline jobs only. Clicking "Create new job" navigates to the job upsert form.

The job detail page (`/jobs/details?id={JOB-ID}`) displays the recent job runs with details as well as general information about the job. Editing or deleting the job is possible from this page via the respective buttons.
