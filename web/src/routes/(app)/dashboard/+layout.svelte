<script lang="ts">
	import * as Sidebar from "$lib/components/sidebar"
	import { goto } from "$app/navigation"
	import type { LayoutProps } from "./$types"
	import Header from "./components/Header.svelte"
	import AppSidebar from "./components/Sidebar.svelte"

	let { data, children }: LayoutProps = $props()

	$effect(() => {
		!data.user && goto("/", { replaceState: true })
	})
</script>

<Sidebar.Provider open={false}>
	<AppSidebar user={data.user} />
	<Sidebar.Inset>
		<Header />
		<div class="flex flex-1 flex-col border bg-black md:rounded-tl-3xl">
			{@render children()}
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
