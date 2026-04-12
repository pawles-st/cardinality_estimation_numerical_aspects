source("common.R")

# datasets config
card <- 50000
mult <- 100
precisions <- c(8)
iters <- 100
transforms <- c("ICDF", "BitHack")

# algorithms to analyze per transform
algorithms <- c("GHLLGeo", "GHLLHar", "GHLLRealGeo", "GHLLRealHar", "GHLLPlus")
algorithms.readable <- c("GHLL (Geo)", "GHLL (Har)", "GHLLReal (Geo)", "GHLLReal (Har)", "GHLL+")

# helper to read with transform
read.data.transformed <- function(alg, transform, prec, card, mult) {
  alg_with_transform <- paste0(alg, "_", transform)
  read.data(alg_with_transform, prec, card, mult)
}

# create the plots
for (prec in precisions) {
  for (trans in transforms) {
    estimates <- list()
    
    # 1. HLL (special case, it doesn't have a transform in name)
    hll.estimates <- read.data("HLL", prec, card, mult)
    estimates <- c(estimates, list(hll.estimates / card))
    
    # 2. Transformed algorithms
    for (i in 1:length(algorithms)) {
      alg.estimates <- read.data.transformed(algorithms[i], trans, prec, card, mult)
      estimates <- c(estimates, list(alg.estimates / card))
    }
    
    # labels
    all.labels <- c("HyperLogLog", algorithms.readable)
    no.algs <- length(all.labels)
    
    # create a comparison boxplot
    png(paste0("boxplot_", trans, "_", prec, ".png"), width = 1920, height = 1080)
    
    par(mar = c(10, 8, 6, 4))
    
    boxplot(estimates,
            main = paste0("Wykres pudełkowy (", trans, ", n = ", format(card, scientific = FALSE), ", k = 2^", prec , ")"),
            xlab = "",
            ylab = "",
            names = all.labels,
            col = rainbow(no.algs),
            cex.main = 3,
            cex.axis = 2,
            las = 2 # Vertical labels
    )
    
    title(xlab = "Algorytm", line = 8, cex.lab = 3)
    title(ylab = "Estymacja / Liczba unikalnych elementów", line = 5, cex.lab = 3)
    
    # mark the means
    means <- lapply(estimates, mean)
    points(unlist(means), pch = 3, cex = 2, lwd = 2)
    
    # mark the cardinality with a line
    abline(h = 1, lwd = 2, lty = 2)
    
    dev.off()
  }
}
