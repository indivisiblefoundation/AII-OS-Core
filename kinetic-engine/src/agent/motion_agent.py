class MotionAgent:
    def choose_variations(self, border_graph, n=100):
        return [{"massing": i*0.15, "wwr": 0.3 + i*0.004} for i in range(n)]
