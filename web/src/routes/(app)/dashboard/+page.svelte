<script lang="ts">
	import Link from "$lib/components/Link.svelte"
	import { apiUrl } from "$lib/utils"
	import type { PageData } from "./$types"

	let { data }: { data: PageData } = $props()
</script>

<div class="flex flex-col">
	{JSON.stringify(data.user)}
	<Link href={`${apiUrl}/auth/logout`}>logout</Link>
</div>
