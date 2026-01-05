<script lang="ts">
	import type { PageProps } from "./$types"

	let { data }: PageProps = $props()
</script>

<svelte:head>
	<title>Projects | Dashboard</title>
	<meta name="description" content="Manage your projects" />
</svelte:head>

{JSON.stringify(data.user)}
