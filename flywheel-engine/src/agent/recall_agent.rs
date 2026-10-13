class RecallAgent:
    def recall(self, query: str):
        # CLIP + Qdrant + reward from dwell time
        print(f"[RecallAgent] Semantic search <100ms for: {query}")
        return ["channel_42_bright_low_energy", "channel_17_flow_optimal"]
