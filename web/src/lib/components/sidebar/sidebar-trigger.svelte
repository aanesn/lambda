<script lang="ts">
	import type { ComponentProps } from "svelte"
	import Sidebar from "$lib/assets/sidebar.svg?raw"
	import Button from "$lib/components/Button.svelte"
	import { useSidebar } from "./context.svelte.js"

	let {
		onclick,
		...restProps
	}: ComponentProps<typeof Button> & {
		onclick?: (e: MouseEvent) => void
	} = $props()

	const sidebar = useSidebar()
</script>

<Button
	intent="ghost"
	size="icon"
	onclick={(e) => {
		onclick?.(e)
		sidebar.toggle()
	}}
	{...restProps}
>
	{@html Sidebar}
</Button>
