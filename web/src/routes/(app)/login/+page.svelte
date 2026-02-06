<script lang="ts">
	import Link from "$lib/components/Link.svelte"
	import { apiUrl } from "$lib/utils"
	import { goto } from "$app/navigation"
	import type { PageData } from "./$types"

	let { data }: { data: PageData } = $props()

	$effect(() => {
		data.user && goto("/dashboard", { replaceState: true })
	})
</script>

<div class="flex flex-col">
	<Link href={`${apiUrl}/auth/google`}>continue with google</Link>
	<Link href={`${apiUrl}/auth/github`}>continue with github</Link>
</div>
