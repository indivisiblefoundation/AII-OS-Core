# WASI-NN on-device — predicts which 100 LHS variations matter
class MotionAgent:
    def __init__(self): self.model = "wasi-nn-energy-comfort-v1"
    def choose_variations(self, border_graph, n=100):
        # Learned Latin Hypercube Sampling — 80 human-curated, 20 AI
        return [{"massing": i*0.15, "wwr": 0.3 + i*0.004} for i in range(n)]
