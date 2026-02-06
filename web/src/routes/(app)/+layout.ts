import type { User } from "$lib/bindings"
import { apiUrl } from "$lib/utils"
import { browser } from "$app/environment"
import type { LayoutLoad } from "../$types"

export const load: LayoutLoad = async ({ fetch }) => {
	if (!browser) return { user: null }
	const res = await fetch(`${apiUrl}/user`, {
		credentials: "include"
	})
	return {
		user: res.ok ? ((await res.json()) as User) : null
	}
}
