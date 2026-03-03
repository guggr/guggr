import { Configuration } from '@/api';
export * from '@/api/gen';

const BASE_PATH = import.meta.env.DEV ? 'http://localhost:8081' : '';

export const config = new Configuration({
	basePath: BASE_PATH,
});
