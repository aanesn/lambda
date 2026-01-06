import { getContext, setContext } from "svelte"
import { IsMobile } from "$lib/utils"

type Getter<T> = () => T

export type SidebarStateProps = {
	open: Getter<boolean>
	setOpen: (open: boolean) => void
}

class SidebarState {
	readonly props: SidebarStateProps
	open = $derived.by(() => this.props.open())
	openMobile = $state(false)
	setOpen: SidebarStateProps["setOpen"]
	#isMobile: IsMobile
	state = $derived.by(() => (this.open ? "expanded" : "collapsed"))
	constructor(props: SidebarStateProps) {
		this.setOpen = props.setOpen
		this.#isMobile = new IsMobile()
		this.props = props
	}
	get isMobile() {
		return this.#isMobile.current
	}
	setOpenMobile = (value: boolean) => {
		this.openMobile = value
	}
	toggle = () => {
		return this.#isMobile.current
			? (this.openMobile = !this.openMobile)
			: this.setOpen(!this.open)
	}
}

const SYMBOL_KEY = "sidebar"

export function setSidebar(props: SidebarStateProps): SidebarState {
	return setContext(Symbol.for(SYMBOL_KEY), new SidebarState(props))
}

export function useSidebar(): SidebarState {
	return getContext(Symbol.for(SYMBOL_KEY))
}
