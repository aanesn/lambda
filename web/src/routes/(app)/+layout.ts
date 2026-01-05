import type { User } from "$lib/bindings"
import { apiUrl } from "$lib/utils"
import type { LayoutLoad } from "./$types"

export const load: LayoutLoad = async ({ fetch }) => {
	let res = await fetch(`${apiUrl}/user`, {
		credentials: "include"
	})
	return {
		user: res.ok ? ((await res.json()) as User) : null
	}
}
