import express from "express";
import tagRouter from "./controller.js";
import { startCronJobs } from "./cron.js";

const app = express();
const PORT = parseInt(process.env.PORT || "3002", 10);

app.use(express.json());
app.use(tagRouter);

app.get("/health", (_req, res) => {
  res.json({ status: "ok", service: "tagging-service" });
});

app.listen(PORT, () => {
  console.log(`tagging-service running on port ${PORT}`);
  startCronJobs();
});
