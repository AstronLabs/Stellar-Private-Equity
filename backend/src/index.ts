import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import router from "./routes";

dotenv.config();

const app = express();
const PORT = process.env.PORT || 4000;

app.use(cors());
app.use(express.json());

// Register API routes
app.use("/api", router);

// Default Route
app.get("/", (req, res) => {
  res.send("Stellar Private Equity Platform Backend is running.");
});

app.listen(PORT, () => {
  console.log(`=================================================`);
  console.log(`🚀 Server running on http://localhost:${PORT}`);
  console.log(`Network: ${process.env.STELLAR_NETWORK || "testnet"}`);
  console.log(`RPC: ${process.env.STELLAR_RPC_URL || "https://soroban-testnet.stellar.org"}`);
  console.log(`=================================================`);
});
