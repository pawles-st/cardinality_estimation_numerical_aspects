# visualisation/defense_boxplot.R

# Config
card <- 500000
size <- card * 100
precisions <- c(4, 8, 12, 16)
iters <- 100

transforms <- c("Naive", "CorrectedBitHack", "BasicBitHack", "LUT", "Spline", "Pade")

# Helper to read results
read_res <- function(alg, prec) {
  # Use sprintf to ensure no scientific notation or extra spaces
  filename <- paste0("../results/", alg, "_", prec, "_", sprintf("%d", card), "_", sprintf("%d", size), ".txt")
  if (file.exists(filename)) {
    vals <- scan(filename, what = double(), nmax = iters, quiet = TRUE)
    return(vals / card)
  }
  return(NULL)
}

for (prec in precisions) {
  # 1. HLL Baseline
  hll_data <- read_res("HLL", prec)

  # 2. GHLL Optimized Variants
  plot_data <- list()
  plot_names <- c()
  plot_colors <- c()

  # Add HLL
  if (!is.null(hll_data)) {
    plot_data[["HLL"]] <- hll_data
    plot_names <- c(plot_names, "HLL")
    plot_colors <- c(plot_colors, "#555555")
  }

  # Transform Metadata (Triadic Palette: Orange, Purple, Green)
  meta <- list(
    # High Precision Pair (Greens)
    list(t="Naive",           d="Naive",           c="#27ae60"), # Nephritis
    list(t="Pade",           d="Pade",           c="#2ecc71"), # Emerald
    
    # BitHacks Pair (Purples)
    list(t="BasicBitHack",          d="BasicBitHack",   c="#9b59b6"), # Amethyst
    list(t="CorrectedBitHack",      d="CorrectedBitHack",  c="#8e44ad"), # Wisteria
    
    # Table-based Pair (Oranges)
    list(t="Spline",         d="Spline",         c="#f39c12"), # Orange
    list(t="LUT",            d="LUT",            c="#e67e22")  # Pumpkin
  )

  for (m in meta) {
    q_data <- read_res(paste0("GHLLPlus_", m$t), prec)
    if (!is.null(q_data)) {
      plot_data[[m$t]] <- q_data
      plot_names <- c(plot_names, m$d)
      plot_colors <- c(plot_colors, m$c)
    }
  }

  if (length(plot_data) == 0) {
    cat(paste0("No data found for precision ", prec, "! Skipping.\n"))
    next
  }

  # Create Plot
  png(paste0("defense_accuracy_boxplot_", prec, ".png"), width = 1600, height = 900, res = 150)
  par(mar = c(6, 6, 4, 2))

  boxplot(plot_data,
          names = NA,
          col = plot_colors,
          main = paste0("Dokładność estymacji (n = ", format(card, scientific = FALSE), ", k = 2^", prec, ")"),
          ylab = "Estymacja / Liczba unikalnych elementów",
          las = 1,
          cex.main = 1.5,
          cex.lab = 1.2,
          border = "gray30",
          xaxt = "n")

  # Custom X-axis labels
  axis(1, at = 1:length(plot_names), labels = FALSE)
  text(x = 1:length(plot_names), 
       y = par("usr")[3] - (par("usr")[4] - par("usr")[3]) * 0.04, 
       labels = plot_names, 
       srt = 0, adj = 0.5, xpd = TRUE, cex = 0.9, font = 2)
  
  title(xlab = "Metoda", line = 4, cex.lab = 1.2)

  # Target line
  abline(h = 1, col = "red", lwd = 2, lty = "dashed")

  grid(nx = NA, ny = NULL, col = "gray90")
  
  # Add a frame (box) around the plot
  box(lwd = 2)

  dev.off()
  cat(paste0("Cleaned accuracy boxplot generated: visualisation/defense_accuracy_boxplot_", prec, ".png\n"))
}

