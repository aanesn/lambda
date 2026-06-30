<script lang="ts">
	import { cx } from "$lib/utils"
	import { DropdownMenu } from "bits-ui"

	let { class: className, ...others }: DropdownMenu.ItemProps = $props()
</script>

<DropdownMenu.Item
	class={cx(
		"focus:bg-neutral-800 gap-2 rounded-xl px-2 py-1.5 text-sm relative flex cursor-default items-center outline-hidden select-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		className
	)}
	{...others}
/>
