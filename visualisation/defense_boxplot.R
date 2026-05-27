# visualisation/defense_boxplot.R

# Config
card <- 500000
size <- card * 100
prec <- 12
iters <- 100

transforms <- c("ICDF", "TaylorBitHack", "BasicBithack", "LUT", "Spline")

# Helper to read results
read_res <- function(alg) {
  # Use sprintf to ensure no scientific notation or extra spaces
  filename <- paste0("../results/", alg, "_", prec, "_", sprintf("%d", card), "_", sprintf("%d", size), ".txt")
  if (file.exists(filename)) {
    vals <- scan(filename, what = double(), nmax = iters, quiet = TRUE)
    return(vals / card)
  }
  return(NULL)
}

# 1. HLL Baseline
hll_data <- read_res("HLL")

# 2. GHLL Optimized Variants (Q16.16 ONLY)
plot_data <- list()
plot_names <- c()
plot_colors <- c()

# Add HLL
if (!is.null(hll_data)) {
  plot_data[["HLL"]] <- hll_data
  plot_names <- c(plot_names, "HLL")
  plot_colors <- c(plot_colors, "#555555")
}

# Transform Metadata
meta <- list(
  list(t="ICDF",           d="ICDF",           c="#3366cc"),
  list(t="BasicBithack",   d="BasicBitHack",   c="#99ccff"),
  list(t="TaylorBitHack",  d="TaylorBitHack",  c="#6699ff"),
  list(t="Spline",         d="Spline",         c="#ffcc66"),
  list(t="LUT",            d="LUT",            c="#ff8c00")
)

for (m in meta) {
  q_data <- read_res(paste0("GHLLGeo_", m$t))
  if (!is.null(q_data)) {
    plot_data[[m$t]] <- q_data
    plot_names <- c(plot_names, m$d)
    plot_colors <- c(plot_colors, m$c)
  }
}

if (length(plot_data) == 0) {
  stop("No data found for plots! Check results directory.")
}

# Create Plot
png("defense_accuracy_boxplot.png", width = 1600, height = 900, res = 150)
par(mar = c(6, 6, 4, 2))

boxplot(plot_data,
        names = NA,
        col = plot_colors,
        main = "Estimation Accuracy",
        ylab = "Estimation / True Cardinality",
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
     srt = 0, adj = 0.5, xpd = TRUE, cex = 1.0, font = 2)

# Target line
abline(h = 1, col = "red", lwd = 2, lty = "dashed")

grid(nx = NA, ny = NULL, col = "gray90")

dev.off()
cat("Cleaned accuracy boxplot generated: visualisation/defense_accuracy_boxplot.png\n")
