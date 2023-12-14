import { redirect, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
	default: async () => {
		throw redirect(303, '/game');
	}
};
