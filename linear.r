sizes <- c(230,230,900,256,242,319,332,326,236,2112,3623,18093)
times_single <- c(273994.2610261026,
                  293962.1916191619,
                  493955.54365436547,
                  298180.60046004603,
                  354556.75757575757,
                  426087.15871587157,
                  525416.4235423543,
                  392088.00150015,
                  327463.68396839686,
                  2205699.5506550656,
                  4668622.622262226,
                  70439084.9090909)
times_multi <- c(870456.2782782783,
                 804176.8778778779,
                 845708.992992993,
                 728760.0950950951,
                 733114.017017017,
                 835310.3383383383,
                 805512.9459459459,
                 750508.9789789789,
                 738256.9719719719,
                 2258491.8698698697,
                 5596881.75075075,
                 56093832.91919192)
single_thread_model = lm(times_single ~ sizes)
multi_thread_model = lm(times_multi ~ sizes)
f1<- function(x){
  return(single_thread_model$coefficients[1]+single_thread_model$coefficients[2]*x)
}
plot(100:30000,f1(100:30000),
     ylab = "Expected time (ns)",
     xlab = "Contract Size (bytes)",
     type = "l",
     col = "blue"
)
abline(multi_thread_model, col = "red")

legend("topleft",
       c("Single Thread","Multi Thread"),
       fill=c("blue","red")
)

