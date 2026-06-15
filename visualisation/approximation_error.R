# approximation_error.R
library(ggplot2)
library(tidyr)
library(dplyr)

# Load data
if (!file.exists("visualisation/all_errors.csv")) {
  stop("File all_errors.csv not found. Run 'cargo run --release --bin gen_all_errors' in gumbel_estimation directory first.")
}

df <- read.csv("visualisation/all_errors.csv")

# Filter for plotting (avoiding extreme ends for clarity)
df_plot <- df %>% filter(q > 0.001 & q < 0.999)

# Define methods metadata
methods <- list(
  list(name = "Naive",           col = "#27ae60", title = "Naive (f32)"),
  list(name = "Pade",           col = "#2ecc71", title = "Pade"),
  list(name = "BasicBitHack",   col = "#9b59b6", title = "Basic BitHack"),
  list(name = "CorrectedBitHack",  col = "#8e44ad", title = "Corrected BitHack"),
  list(name = "Spline",         col = "#f39c12", title = "Spline (Q16.16)"),
  list(name = "LUT",            col = "#e67e22", title = "LUT (Q6.2)")
)

# Shared theme for high visibility and centered titles
vis_theme <- theme_minimal() +
  theme(
    plot.title = element_text(size = 22, face = "bold", hjust = 0.5),
    plot.subtitle = element_text(size = 16, hjust = 0.5),
    axis.title = element_text(size = 18),
    axis.text = element_text(size = 14),
    legend.position = "bottom",
    legend.text = element_text(size = 14),
    panel.border = element_rect(colour = "black", fill = NA, linewidth = 1)
  )

for (m in methods) {
  p <- ggplot(df_plot, aes(x = q, y = .data[[m$name]])) +
    geom_line(color = m$col, linewidth = 1.0) +
    vis_theme +
    labs(title = paste0("Błąd bezwzględny: ", m$title),
         subtitle = "Porównanie z wartością teoretyczną (f64)",
         x = "Kwantyl (q)",
         y = "Błąd bezwzględny")
  
  filename <- paste0("plots/error_", m$name, "_standalone.png")
  ggsave(filename, p, width = 10, height = 7, dpi = 300)
  cat(paste("Generated:", filename, "\n"))
}

cat("All individual error plots updated with centered titles and improved visibility.\n")
