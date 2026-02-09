<script lang="ts">
	import * as Sidebar from "$lib/components/sidebar"
	import { goto } from "$app/navigation"
	import type { LayoutProps } from "./$types"
	import Header from "./components/Header.svelte"
	import AppSidebar from "./components/Sidebar.svelte"

	let { data, children }: LayoutProps = $props()

	$effect(() => {
		!data.user && goto("/login", { replaceState: true })
	})
</script>

<Sidebar.Provider>
	<AppSidebar user={data.user} />
	<Sidebar.Inset>
		<Header />
		<div class="flex-1 border-t bg-neutral-950 p-6 md:rounded-tl-4xl md:border-s">
			{@render children()}
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
