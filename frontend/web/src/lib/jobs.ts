/** @returns The job name from the job's type ID or `'Unknown'` */
export function getJobName(jobType: string) {
	if (jobType === 'ping') return 'Ping';
	if (jobType === 'http') return 'HTTP';

	return 'Unknown';
}
