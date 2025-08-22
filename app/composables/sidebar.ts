export function useSidebar() {
  const showSidebar = useState('showSidebar', () => false)

  return {
    showSidebar,
  }
}
