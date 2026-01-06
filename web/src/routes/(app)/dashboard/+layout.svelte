<script lang="ts">
	import type { LayoutProps } from "./$types"
	import { goto } from "$app/navigation"
	import * as Sidebar from "$lib/components/sidebar"
	import AppSidebar from "./components/Sidebar.svelte"
	import Header from "./components/Header.svelte"

	let { data, children }: LayoutProps = $props()

	$effect(() => {
		!data.user && goto("/", { replaceState: true })
	})
</script>

<Sidebar.Provider open={false}>
	<AppSidebar user={data.user} />
	<Sidebar.Inset>
		<Header />
		{@render children()}
	</Sidebar.Inset>
</Sidebar.Provider>
